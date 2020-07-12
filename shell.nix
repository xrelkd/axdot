with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "axdot-dev";

  RUST_BACKTRACE = 1;

  nativeBuildInputs = [ rustup cargo-make ];

}
