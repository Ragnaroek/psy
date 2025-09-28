(include "gb_dma")

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

('tile-data db 0)
; TODO define the tile data
