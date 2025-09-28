(include "gb_dma") ; loads all the stuff for the dma-gameboy, especially a lot of macros to set the proper gameboy stuff
;; include def ideas

(def-section .sram
   :offset 0xa000
   :length 100 ; in bytes
   :label-only true ;memory may only be references and nothing be placed in this memory block
)

(def-section .rom0
    :offset 0x150
    :length 100 ; in bytes
    :label-only false)

;; end include code

(section .sram)
('value1 db)
('value2 db)
('value3 db)

(section .rom0)
('main ld 0x00 0xa)
(ld 'value1 666)
(ld 'value2 666)
(ld 'value3 666)
;('value4 db 1) ;TODO db with value in non label-section

(sub-section 200)
