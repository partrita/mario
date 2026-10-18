# Mario Terminal Man Pages

This directory contains manual pages for Mario terminal emulator in scdoc format.

## Files

- `mario.1.scd` - Main Mario terminal manual page (section 1)
- `mario.5.scd` - Mario configuration file format manual page (section 5)  
- `mario-bindings.5.scd` - Mario key bindings manual page (section 5)

## Building

To build the man pages, you need `scdoc` installed:

### Install scdoc

**macOS (Homebrew):**
```bash
brew install scdoc
```

**Ubuntu/Debian:**
```bash
sudo apt install scdoc
```

**Arch Linux:**
```bash
sudo pacman -S scdoc
```

**From source:**
```bash
git clone https://git.sr.ht/~sircmpwn/scdoc
cd scdoc
make
sudo make install
```

### Build man pages

```bash
# Build all man pages
make -C extra/man

# Or build individually
scdoc < extra/man/mario.1.scd > mario.1
scdoc < extra/man/mario.5.scd > mario.5
scdoc < extra/man/mario-bindings.5.scd > mario-bindings.5
```

### Install man pages

```bash
# Install to system man directory (requires sudo)
sudo cp mario.1 /usr/local/share/man/man1/
sudo cp mario.5 /usr/local/share/man/man5/
sudo cp mario-bindings.5 /usr/local/share/man/man5/

# Update man database
sudo mandb
```

### View man pages

```bash
man mario
man 5 mario
man 5 mario-bindings
```

## Format

The man pages are written in scdoc format, which is a simple markup language for writing man pages. See the [scdoc documentation](https://git.sr.ht/~sircmpwn/scdoc) for syntax details.