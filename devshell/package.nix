{ name
, version
, lib
, rustPlatform
, installShellFiles
}:

rustPlatform.buildRustPackage {
  pname = name;
  inherit version;

  src = lib.cleanSource ./..;

  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [ installShellFiles ];

  postInstall = ''
    installShellCompletion --cmd axdot \
      --bash <($out/bin/axdot completions bash) \
      --fish <($out/bin/axdot completions fish) \
      --zsh  <($out/bin/axdot completions zsh)
  '';

  meta = with lib; {
    homepage = "https://github.com/xrelkd/axdot";
    license = with licenses; [ gpl3 ];
    maintainers = with maintainers; [ xrelkd ];
  };

}
