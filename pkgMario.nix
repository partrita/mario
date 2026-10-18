{
  # rust-overlay deps
  rust-toolchain,
  makeRustPlatform,
  # Normal deps
  lib,
  stdenv,
  darwin,
  autoPatchelfHook,
  cmake,
  ncurses,
  pkg-config,
  gcc-unwrapped,
  fontconfig,
  libGL,
  vulkan-loader,
  libxkbcommon,
  withX11 ? !stdenv.isDarwin,
  libX11,
  libXcursor,
  libXi,
  libXrandr,
  libxcb,
  withWayland ? !stdenv.isDarwin,
  wayland,
  shaderc,
  ...
}: let
  readTOML = f: builtins.fromTOML (builtins.readFile f);
  cargoToml = readTOML ./Cargo.toml;
  rioToml = readTOML ./frontends/marioterm/Cargo.toml;
  rustPlatform = makeRustPlatform {
    cargo = rust-toolchain;
    rustc = rust-toolchain;
  };
  rlinkLibs =
    lib.optionals stdenv.isLinux [
      (lib.getLib gcc-unwrapped)
      fontconfig
      libGL
      libxkbcommon
      vulkan-loader
    ]
    ++ lib.optionals withX11 [
      libX11
      libXcursor
      libXi
      libXrandr
      libxcb
    ]
    ++ lib.optionals withWayland [
      wayland
    ];

  inherit (lib.fileset) unions toSource;
in
  rustPlatform.buildRustPackage {
    inherit (cargoToml.workspace.package) version;
    name = "mario";
    src = toSource {
      root = ./.;
      fileset = unions ([
          ./Cargo.lock
          ./Cargo.toml
          ./misc # Extra desktop/terminfo files
        ]
        ++ (map (x: ./. + "/${x}") cargoToml.workspace.members));
    };
    cargoLock.lockFile = ./Cargo.lock;

    cargoBuildFlags = "-p marioterm";

    buildInputs = rlinkLibs ++ (lib.optionals stdenv.isDarwin [darwin.libutil]);
    runtimeDependencies = rlinkLibs;

    nativeBuildInputs =
      [
        rustPlatform.bindgenHook
        ncurses
        shaderc
      ]
      ++ lib.optionals stdenv.isLinux [
        cmake
        pkg-config
        autoPatchelfHook
      ];

    outputs = [
      "out"
      "terminfo"
    ];

    postInstall =
      ''
        install -D -m 644 misc/mario.desktop -t \
                          $out/share/applications
        install -D -m 644 misc/logo.svg \
                          $out/share/icons/hicolor/scalable/apps/mario.svg

        # Install terminfo files
        install -dm 755 "$terminfo/share/terminfo/r/"
        tic -xe xterm-mario,mario,mario-direct -o "$terminfo/share/terminfo" misc/mario.terminfo
        mkdir -p $out/nix-support
        echo "$terminfo" >> $out/nix-support/propagated-user-env-packages
      ''
      + lib.optionalString stdenv.hostPlatform.isDarwin ''
        mkdir $out/Applications/
        mv misc/osx/Mario.app/ $out/Applications/
        mkdir $out/Applications/Mario.app/Contents/MacOS/
        ln -s $out/bin/mario $out/Applications/Mario.app/Contents/MacOS/
      '';

    buildNoDefaultFeatures = true;
    buildFeatures = (lib.optionals withX11 ["x11"]) ++ (lib.optionals withWayland ["wayland"]);
    checkType = "debug";
    meta = {
      description = rioToml.package.description;
      longDescription = rioToml.package.extended-description;
      homepage = cargoToml.workspace.package.homepage;
      license = lib.licenses.mit;
      platforms = lib.platforms.unix;
      changelog = "https://github.com/raphamorim/mario/blob/master/CHANGELOG.md";
      mainProgram = "mario";
    };
  }
