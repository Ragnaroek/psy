; a psy port of the gbdev.io tutorial example: https://gbdev.io/gb-asm-tutorial/assets/hello-world.asm
(include :std "gb/dma")

(section .header)
(jp 'rom0)

(section .rom0)
; Shut down audio circuitry
(ld %a 0) ;0x150
(ld ('hw-sound) %a) ;0x152

; Do not turn the LCD off outside of VBlank
(label 'wait-vb-blank) ;0x155
(ld %a ('hw-ly))
(cp 144)
(jp #c 'wait-vb-blank)

; Turn the LCD off
(ld %a 0)
(ld ('hw-lcdc) %a)

; Copy the tile data
(ld %de 'tiles)
(ld %hl 0x9000)
(ld %bc (- 'tiles-end 'tiles))

(label 'tiles)
(db 0x00 0xff 0x00 0xff 0x00 0xff 0x00 0xff 0x00 0xff 0x00 0xff 0x00 0xff 0x00 0xff)
; TODO define remaining tiles data
(label 'tiles-end)

(label 'tilemap)
; TODO define tilemap
(label 'tilemap-end)
