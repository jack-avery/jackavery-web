---
title: one month with nixos
date: 2024-12-04
toc: false
draft: false
tags:
  - linux
---

it was now a month ago i made the swap to [NixOS](https://nixos.org/),
and i've passed my initial honeymoon phase with it. that isn't to say NixOS isn't a gorgeous distro,
and especially not to say that nixpkgs is unsatisfactory, but i find that i'm beginning to see the seams.

my layman's description of NixOS as I understand it right now is that it's a linux distribution
which uses the nix package manager as its' primary package manager. it also provides the ability to
**declaratively** manage the packages and configuration of the system.
for enterprise application, such as distributed laptops to employees, this sounds wonderful,
and for managing development environments, it has proven a very wonderful tool.

**however...**

NixOS has a preference that everything on the system be managed through the system configuration file.
i understand the appeal, but seeing as i'm not swapping systems that often, i opted to forego all of that.
the reproducibility of NixOS is definitely more geared towards enterprise application,
instead of home personal computing. thankfully, this decision hasn't affected my experience on NixOS;
the system seems perfectly fine with you managing things externally from the system config.

another thing to note is that NixOS will outright refuse to run dynamically linked binaries not a part of the
Nix store; that is, any application that isn't statically compiled not managed by Nix. this makes sense considering
a quick peek into the /lib folder confirms it is completely empty, except a symlink to ld-linux.so.2, which i assume
is there for compatibility reasons.

this normally wouldn't be such an issue; just incorporate the package into Nix. the issue arises then that learning
how to package applications that **you did not make** in Nix is kind-of difficult. some applications
can have special handling for certain dependencies, and that special handling can in turn become difficult to
manage, especially if it expects to write to the Nix store; which is intended to be immutable, and read-only.

**despite this...**

i have discovered that Nix itself is an extremely powerful tool for creating build scripts; i now use it for
[ansible-tf2network's relay bot](https://github.com/jack-avery/ansible-tf2network-relay),
as well as a simple development flake that handles dependencies and an ssh agent for me.

and my favorite part: Nix, the package manager, is almost entirely distro-agnostic, allowing you to install it on
pretty much any linux distribution and benefit from the wealth of packages that nixpkgs contains.

i will likely continue using NixOS as it has proven to be a stable and reliable system, but sometimes i yearn for
the often deeper, imperative control of something like Gentoo; but no distro is perfect, i suppose, and we all
have to accept the benefits and pitfalls. NixOS will still be my home for the foreseeable future.