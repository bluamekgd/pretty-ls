# pretty ls

A pretty ls replacement, inspired by the nushell ls.

## Run with Nix

```nix
nix run github:BluamekGD/pretty-ls
```

This compiles and runs the latest source.

Nothing gets installed on your system. If you wanna install it, see the instructions below.

## NixOS

1. Add this input to your flake.

```nix
pretty-ls = {
  url = "github:BluamekGD/pretty-ls";
  inputs.nixpkgs.follows = "nixpkgs";
};
```

2. Add to system (or home) packages

```nix
environment.systemPackages = with pkgs; [
  inputs.pretty-ls.packages.${pkgs.stdenv.hostPlatform.system}.default
];
```

3. Rebuild NixOS

## Other Linux distributions

Get the binary from the [releases](https://github.com/BluamekGD/pretty-ls/releases/latest) page.

## Windows

Not supported, sorry!

## macOS

There's no official support but since macOS is a Unix-based system it ***might*** work. Try it but don't expect it to work perfectly.

#

### Note: I used Claude for sorting and debugging so if y'all care about that then keep that in mind.
