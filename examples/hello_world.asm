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
