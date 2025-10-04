; non trivial psy gb program
; ported from: https://github.com/tbsp/simple-gb-asm-examples/blob/master/src/background-tile/background-tile.asm

(include :std "gb/dma")

(section .header)
(jp 'entry-point)

(section .rom0)
(ld %hl 'tile-data)
(ld %de 'vram)
(ld %b 32)

('copy-loop ld %a (%hl))
(ld (%de) %a)
(inc %hl)
(inc %de)
(dec %b)
(jr #nz 'copy-loop)

(ld %hl 'tile-map-0)
(ld (%hl) 1)
(inc %hl)

('loop-forever jr 'loop-forever)

(label 'tile-data)
(db 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0)

(label 'block)
(db 0b00000000 0b11111111)
(db 0b01000010 0b10000001)
(db 0b00000000 0b11111111)
(db 0b01000010 0b10000001)
(db 0b00000000 0b11111111)
(db 0b01000010 0b10000001)
(db 0b00000000 0b11111111)
(db 0b11111111 0b11111111)
