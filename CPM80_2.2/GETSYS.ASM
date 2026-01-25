;--------------------------------------------------------------------
; 
; GETSYS - reads system tracks into memory at 3380H
;
; based on getsys/putsys in D.R. manuals, configured for the
; Digital Research Computers of Texas (Ferguson) Big-Board 
;      Brian Kantor, 1983
;--------------------------------------------------------------------
; Boot loader	image starts at 3380H
; CCP		image starts at 3400H
; BDOS		image starts at 3C00H
; BIOS		image starts at 4000H
; (end of system track image is end of bios at 4FFFH
;-------------------------------------------------------------------

msize	equ	60			; size of cp/m in kbytes

bias	equ	(msize-20)*1024
ccp	equ	3400h+bias
bdos	equ	ccp+800h
bios	equ	ccp+1600h

	org	100h
getsys:
	mvi	c,1			;select the b disk
	call	bios+27			;select
	lxi	h,3380h			;convenient place
	mvi	b,0			;start on track 0
rd_trk:
	mvi	c,1			;sector 1
rd_sec:
	call	read_sec		;get a sector
	lxi	d,128			;sector length
	dad	d			;put pointer at end of sector
	inr	c			;bump count
	mov	a,c			;check if we got 26 yet
	cpi	27			;26 on a track
	jnz	rd_sec

; at end of track, get next one

	inr	b			;bump track count
	mov	a,b			;check count
	cpi	2			;done 0 and 1 yet?
	jnz	rd_trk			;no, do another

; done, so boot system

	ret				;return to cp/m
;
; subroutine to read a sector (use the bios)
;

read_sec:
	push	h
	push	b
	mov	b,h			; set dma address
	mov	c,l
	call	bios+36			;setdma
	pop	b

	push	b
	mov	c,b			; set track number
	mvi	b,0
	call	bios+30
	pop	b

	push	b
	mvi	b,0
	call	bios+33			; set sector number
	call	bios+39			; sector read
	pop	b
	pop	h
	ret

	end
	