(include "gb-dma") ; loads all the stuff for the dma-gameboy, especially a lot of macros to set the proper gameboy stuff
;; include def ideas

(def-section .sram
   :offset 0xa000
   :length xxxx
   :label-only true ;memory may only be references and nothing be placed in this memory block
)

(def-section .rom0
    :offset 0x150
    :length xxxx
    :label-only false)

;; end include code

(section .sram)
(db :value1)
(db :value2)
(db :value2)

(section .rom0)
(label 'main) ; same as below
('main ld 0x00 0xa)
(ld :value1 666)
(ld :value2 666)
(ld :value3 666)

(sub-section 200)
