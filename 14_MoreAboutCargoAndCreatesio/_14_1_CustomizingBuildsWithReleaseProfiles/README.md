# Customizing Builds with Release Profiles

In Rust, release profiles are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options of compiling code.Each profile is configured indenpendently of the others.

Cargo has two main profiles:

 the `dev` profile, Cargo uses when you run `cargo build` and the `release` profile, which Cargo uses when you run `cargo build -- release`.

- the `dev` profile is defined with good defaults for development
- the `release` profile has good defaults for release builds.


Cargo has default setting for each profile that apply when you have not explicitly set any profile.`[profile.*]` section in the `Cargo.toml` file.

a example:

```
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

the opt-level setting controls the level of optimization Rust will apply to your code, with a range of 0 to 3.
Applying more optimizations extends compiling time, so if you in development and compiling your code often, you'll want fewer optimizations to compile faster event the resulting code runs slower.

You can override a default setting by adding a different value for it in Cargo.toml. For example, if we want to use optimization level 1 in the development profile, we can add these two lines to our project’s Cargo.toml file:

```
[profile.dev]
opt-level = 1
```

