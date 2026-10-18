# Man page generation
.PHONY: all clean install

SCDOC = scdoc
MANDIR = /usr/local/share/man

all: mario.1 mario.5 mario-bindings.5

mario.1: mario.1.scd
	$(SCDOC) < $< > $@

mario.5: mario.5.scd
	$(SCDOC) < $< > $@

mario-bindings.5: mario-bindings.5.scd
	$(SCDOC) < $< > $@

install: all
	mkdir -p $(DESTDIR)$(MANDIR)/man1 $(DESTDIR)$(MANDIR)/man5
	install -m644 mario.1 $(DESTDIR)$(MANDIR)/man1/mario.1
	install -m644 mario.5 $(DESTDIR)$(MANDIR)/man5/mario.5
	install -m644 mario-bindings.5 $(DESTDIR)$(MANDIR)/man5/mario-bindings.5

clean:
	rm -f mario.1 mario.5 mario-bindings.5