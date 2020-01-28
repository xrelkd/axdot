{ lib
, rustPlatform
}:

rustPlatform.buildRustPackage rec {
  pname = "axdot";
  version = "0.2.0";

  src = lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;

  meta = with lib; {
    homepage = "https://github.com/xrelkd/axdot";
    license = with licenses; [ gpl3 ];
    maintainers = with maintainers; [ xrelkd ];
  };
}
