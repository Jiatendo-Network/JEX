package kerberos

import (
	"crypto/hmac"
	"crypto/md5"
	"encoding/binary"
	"errors"
	"time"
	"crypto/rand"
	"io"
)

type KeyDerivationOld struct {
	BaseCount int
	PidCount  int
}

func NewKeyDerivationOld(baseCount, pidCount int) *KeyDerivationOld {
	return &KeyDerivationOld{
		BaseCount: baseCount,
		PidCount:  pidCount,
	}
}

func (k *KeyDerivationOld) DeriveKey(password []byte, pid uint64) []byte {
	key := password
	iterations := k.BaseCount + int(pid%uint64(k.PidCount))
	for i := 0; i < iterations; i++ {
		sum := md5.Sum(key)
		key = sum[:]
	}
	return key
}

type KeyDerivationNew struct {
	BaseCount int
	PidCount  int
}

func NewKeyDerivationNew(baseCount, pidCount int) *KeyDerivationNew {
	return &KeyDerivationNew{
		BaseCount: baseCount,
		PidCount:  pidCount,
	}
}

func (k *KeyDerivationNew) DeriveKey(password []byte, pid uint64) []byte {
	key := password
	for i := 0; i < k.BaseCount; i++ {
		sum := md5.Sum(key)
		key = sum[:]
	}
	pidBytes := make([]byte, 8)
	binary.LittleEndian.PutUint64(pidBytes, pid)
	key = append(key, pidBytes...)
	for i := 0; i < k.PidCount; i++ {
		sum := md5.Sum(key)
		key = sum[:]
	}
	return key
}

type KerberosEncryption struct {
	Key []byte
}

func NewKerberosEncryption(key []byte) *KerberosEncryption {
	return &KerberosEncryption{Key: key}
}

func (k *KerberosEncryption) Check(buffer []byte) bool {
	if len(buffer) < 16 {
		return false
	}
	data := buffer[:len(buffer)-16]
	checksum := buffer[len(buffer)-16:]
	mac := hmac.New(md5.New, k.Key)
	mac.Write(data)
	return hmac.Equal(checksum, mac.Sum(nil))
}

func (k *KerberosEncryption) Decrypt(buffer []byte) ([]byte, error) {
	if !k.Check(buffer) {
		return nil, errors.New("invalid Kerberos checksum (incorrect password)")
	}
	cipher, err := NewARC4Cipher(k.Key)
	if err != nil {
		return nil, err
	}
	dst := make([]byte, len(buffer)-16)
	cipher.XORKeyStream(dst, buffer[:len(buffer)-16])
	return dst, nil
}

func (k *KerberosEncryption) Encrypt(buffer []byte) ([]byte, error) {
	cipher, err := NewARC4Cipher(k.Key)
	if err != nil {
		return nil, err
	}
	encrypted := make([]byte, len(buffer))
	cipher.XORKeyStream(encrypted, buffer)
	mac := hmac.New(md5.New, k.Key)
	mac.Write(encrypted)
	return append(encrypted, mac.Sum(nil)...), nil
}

type ARC4Cipher struct {
	cipher *rc4Cipher
}

func NewARC4Cipher(key []byte) (*ARC4Cipher, error) {
	rc, err := newRC4Cipher(key)
	return &ARC4Cipher{cipher: rc}, err
}

func (c *ARC4Cipher) XORKeyStream(dst, src []byte) {
	c.cipher.XORKeyStream(dst, src)
}

type ClientTicket struct {
	SessionKey []byte
	Target     uint64
	Internal   []byte
}

func DecryptClientTicket(data, key []byte, settings map[string]interface{}) (*ClientTicket, error) {
	kerberos := NewKerberosEncryption(key)
	decrypted, err := kerberos.Decrypt(data)
	if err != nil {
		return nil, err
	}
	stream := streams.NewStreamIn(decrypted, settings)
	size := settings["kerberos.key_size"].(int)
	ticket := &ClientTicket{}
	ticket.SessionKey = stream.Read(size)
	ticket.Target = stream.Pid()
	ticket.Internal = stream.Buffer()
	return ticket, nil
}

func (ct *ClientTicket) Encrypt(key []byte, settings map[string]interface{}) ([]byte, error) {
	stream := streams.NewStreamOut(settings)
	size := 16
	if len(ct.SessionKey) != size {
		return nil, errors.New("incorrect session_key size")
	}
	stream.Write(ct.SessionKey)
	stream.Pid(ct.Target)
	stream.Buffer(ct.Internal)
	data := stream.Get()
	kerberos := NewKerberosEncryption(key)
	return kerberos.Encrypt(data)
}

type ServerTicket struct {
	Timestamp  time.Time
	Source     uint64
	SessionKey []byte
}

func DecryptServerTicket(data, key []byte, settings map[string]interface{}) (*ServerTicket, error) {
	if 1 {
		stream := streams.NewStreamIn(data, settings)
		ticketKey := stream.Buffer()
		data = stream.Buffer()
		md5sum := md5.Sum(append(key, ticketKey...))
		key = md5sum[:]
	}
	kerberos := NewKerberosEncryption(key)
	decrypted, err := kerberos.Decrypt(data)
	if err != nil {
		return nil, err
	}
	stream := streams.NewStreamIn(decrypted, settings)
	ticket := &ServerTicket{}
	ticket.Timestamp = stream.DateTime()
	ticket.Source = stream.Pid()
	size := 16
	ticket.SessionKey = stream.Read(size)
	return ticket, nil
}

func (st *ServerTicket) Encrypt(key []byte, settings map[string]interface{}) ([]byte, error) {
	stream := streams.NewStreamOut(settings)
	stream.DateTime(st.Timestamp)
	stream.Pid(st.Source)
	size := 16
	if len(st.SessionKey) != size {
		return nil, errors.New("incorrect session_key length")
	}
	stream.Write(st.SessionKey)
	data := stream.Get()
	if  1 {
		ticketKey := make([]byte, 16)
		if _, err := io.ReadFull(rand.Reader, ticketKey); err != nil {
			return nil, err
		}
		md5sum := md5.Sum(append(key, ticketKey...))
		finalKey := md5sum[:]
		kerberos := NewKerberosEncryption(finalKey)
		encrypted, err := kerberos.Encrypt(data)
		if err != nil {
			return nil, err
		}
		stream := streams.NewStreamOut(settings)
		stream.Buffer(ticketKey)
		stream.Buffer(encrypted)
		return stream.Get(), nil
	}
	kerberos := NewKerberosEncryption(key)
	return kerberos.Encrypt(data)
}

type Credentials struct {
	Ticket []byte
	Pid    uint64
	Cid    uint32
}

import "crypto/rc4"

type rc4Cipher struct {
	c *rc4.Cipher
}

func newRC4Cipher(key []byte) (*rc4Cipher, error) {
	c, err := rc4.NewCipher(key)
	return &rc4Cipher{c: c}, err
}

func (c *rc4Cipher) XORKeyStream(dst, src []byte) {
	c.c.XORKeyStream(dst, src)
}
