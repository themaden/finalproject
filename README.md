# Soroban Token Sözleşmesi

Bu proje, Stellar blok zincirinde çalışan Soroban smart contract platformu için geliştirilmiş bir token sözleşmesidir.

## Genel Bakış

Soroban Token Sözleşmesi, temel token işlevleri sağlayan kapsamlı bir akıllı sözleşmedir. Bu sözleşme ile token oluşturma, transfer etme, yakma ve üçüncü taraf harcamalarını yetkilendirme gibi standart token işlemlerini gerçekleştirebilirsiniz.

## Özellikler

* **Token Yönetimi**: Basma, yakma ve transfer işlemleri
* **Yetkilendirme**: Üçüncü taraf harcamaları için izin mekanizması
* **Yönetici Kontrolü**: Yönetici atama ve yönetici izni gerektiren işlemler
* **Meta Veri**: Token adı, sembol ve ondalık bilgileri

## Başlangıç

### Gereksinimler

* Rust programlama dili ([rust-lang.org](https://www.rust-lang.org/))
* Soroban SDK 22.0.7 veya daha yeni
* Cargo (Rust paket yöneticisi)

### Kurulum

1. Bu depoyu klonlayın:
```bash
git clone https://github.com/kullanici-adi/soroban-token.git
cd soroban-token
```

2. Rust yükleyin (eğer yüklü değilse):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. WebAssembly hedefini ekleyin:
```bash
rustup target add wasm32-unknown-unknown
```

4. Soroban CLI'ı yükleyin:
```bash
cargo install --locked soroban-cli
```

### Derleme

Sözleşmeyi derlemek için:

```bash
cargo build --target wasm32-unknown-unknown --release
```

Bu, sözleşmenizin WebAssembly (WASM) sürümünü `target/wasm32-unknown-unknown/release/soroban_token.wasm` konumunda oluşturacaktır.

### Test Etme

Sözleşme birim testlerini çalıştırmak için:

```bash
cargo test
```

## Kullanım

### Yerel Test

Soroban CLI kullanarak sözleşmeyi yerel ortamda test edebilirsiniz:

```bash
# Sözleşmeyi başlat
soroban contract invoke \
  --wasm target/wasm32-unknown-unknown/release/soroban_token.wasm \
  --id 1 \
  -- \
  initialize \
  --admin <ADMIN_ADRESI> \
  --name "Soroban Token" \
  --symbol "SRT" \
  --decimals 8

# Token bas
soroban contract invoke \
  --wasm target/wasm32-unknown-unknown/release/soroban_token.wasm \
  --id 1 \
  -- \
  mint \
  --admin <ADMIN_ADRESI> \
  --to <ALICI_ADRESI> \
  --amount 1000000000

# Bakiye sorgula
soroban contract invoke \
  --wasm target/wasm32-unknown-unknown/release/soroban_token.wasm \
  --id 1 \
  -- \
  balance \
  --addr <ADRES>
```

### Testnet Dağıtımı

Sözleşmeyi Stellar Testnet'e dağıtmak için:

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/soroban_token.wasm \
  --source <KAYNAK_ADRESI> \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase 'Test SDF Network ; September 2015'
```

## API Referansı

### Token Yönetimi Fonksiyonları

* `initialize(admin, name, symbol, decimals)`: Sözleşmeyi başlatır
* `mint(admin, to, amount)`: Yeni tokenler oluşturur (yalnızca admin)
* `burn(from, amount)`: Tokenleri yakar
* `transfer(from, to, amount)`: Tokenleri transfer eder
* `balance(addr)`: Adres bakiyesini görüntüler
* `total_supply()`: Toplam arzı görüntüler

### Ödenek Yönetimi Fonksiyonları

* `approve(from, spender, amount)`: Harcama izni verir
* `allowance(from, spender)`: İzin miktarını görüntüler
* `transfer_from(spender, from, to, amount)`: İzinli token transferi yapar

### Yönetim Fonksiyonları

* `set_admin(admin, new_admin)`: Yönetici adresini değiştirir
* `get_metadata()`: Token meta verilerini döndürür

## Sözleşme Mimarisi

Sözleşme, aşağıdaki önemli bileşenleri içerir:

* **TokenMetadata Yapısı**: Token meta verilerini (ad, sembol, ondalıklar) saklar
* **DataKey Enum**: Sözleşme depolama anahtarlarını tanımlar
* **TokenContract Yapısı**: Temel sözleşme işlevlerini uygular

## Katkıda Bulunma

Katkılarınızı memnuniyetle karşılıyoruz! Lütfen bir pull request göndermeden önce testlerinizi çalıştırdığınızdan emin olun.

## Lisans

Bu proje [MIT Lisansı](LICENSE) altında lisanslanmıştır.

## İletişim

Sorularınız veya geri bildirimleriniz için [info@example.com](mailto:info@example.com) adresine e-posta gönderebilirsiniz.
