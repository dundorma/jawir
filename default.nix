with import <nixpkgs> { };

rustPlatform.buildRustPackage {
  pname = "jawir";
  version = "0.1.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    glib
    openssl
  ];

  PKG_CONFIG_PATH = "${glib.dev}/lib/pkgconfig:${openssl.dev}/lib/pkgconfig";
  LD_LIBRARY_PATH = lib.makeLibraryPath [ openssl ];
}
