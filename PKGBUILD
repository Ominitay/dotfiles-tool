pkgname=dotfiles-tool
pkgver=0.1.0
pkgrel=1
makedepends=('rust' 'cargo' 'git')
arch=('i686' 'x86_64' 'armv6h' 'armv7h' 'aarch64')

build() {
    cargo build --release --target-dir=target
}

package() {
    install -Dm 755 target/release/${pkgname} -t "${pkgdir}/usr/bin"
}
