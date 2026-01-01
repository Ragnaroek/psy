(def-section .tile-map-0
    :offset 0x9800
    :label-only true)
(section .tile-map-0)
('tile-map-0 db)

(def-section .tile-map-1
    :offset 0x9C00
    :label-only true)

(def-section .header
    :offset 0x100
    :length 0x50)

; a ROM has at least to 16 KiB Banks, always named here
; rom0 and rom1
(def-section .rom0
    :offset 0x150
    :length 0x4000) ; 16 KiB
(section .rom0)
('rom0 db)

(def-section .rom1
    :offset 0x4150
    :length 0x4000) ; 16 KiB
(section .rom1)
('rom1 db)

(def-section .vram
    :offset 0x8000
    :length 0x1FFF
    :label-only true)
(section .vram)
('vram db)

(def-section .hw-ports
    :offset 0xFF00
    :length 0x80
    :label-only true)
(section .hw-ports)
('hw-joyp db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
(db)
('hw-sound db) ; aka NR52, 0xFF26
(db) ;unused 0xFF27
(db) ;unused 0xFF28
(db) ;unused 0xFF29
(db) ;unused 0xFF2A
(db) ;unused 0xFF2B
(db) ;unused 0xFF2C
(db) ;unused 0xFF2D
(db) ;unused 0xFF2E
(db) ;unused 0xFF2F
(db) ;0xFF30
(db) ;0xFF31
(db) ;0xFF32
(db) ;0xFF33
(db) ;0xFF34
(db) ;0xFF35
(db) ;0xFF36
(db) ;0xFF37
(db) ;0xFF38
(db) ;0xFF39
(db) ;0xFF3A
(db) ;0xFF3B
(db) ;0xFF3C
(db) ;0xFF3D
(db) ;0xFF3E
(db) ;0xFF3F
(db) ;0xFF40
(db) ;0xFF41
(db) ;0xFF42
(db) ;0xFF43
('hw-ly db) ; 0xFF44
