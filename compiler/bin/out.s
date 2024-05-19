	.text
	.global _main

_main:
	
	sub sp, sp, #16
	
	mov w0, #10
	str w0, [sp,12]
	mov w0, 0
	add sp, sp, 16
	
	
	mov w0, #1
	sub sp, sp, #16
	str w0, [sp, 12]

	mov w0, #2
	ldr w1, [sp, 12]
	sub w0, w1, w0
	add sp, sp, 16
	sub sp, sp, 16
	str w0, [sp, 12]
	
	mov w0, #2
	ldr w1, [sp, 12]
	cmp w1, w0
	bgt .L2
	mov w0, 0
	 b .L3
.L2:
	mov w0, 1
.L3:
	add sp, sp, 16
	sub sp, sp, 16
	str w0, [sp, 12]
	
	mov w0, #3
	ldr w1, [sp, 12]
	cmp w1, w0
	blt .L4
	mov w0, 0
	b .L5
.L4:
	mov w0, 1
.L5:
	add sp, sp, 16
	
	sub sp, sp, #16
	str w0, [sp, 16]
	
	mov w0, #1
	ldr w1, [sp, 16]
	cmp w1, w0
	bge .L6
	mov w0, 0
	b .L7
.L6:
	mov w0, 1
.L7:
	add sp, sp, 16
	sub sp, sp, #16
	str w0, [sp, 16]
	
	mov w0, #4
	sub sp, sp, #16
	str w0, [sp, 12]
	mov w0, #4
	sub sp, sp, #16
	str w0, [sp, 12]
	
	mov w0, #4
	ldr w1, [sp, 12]
	sdiv w0, w1, w0
	add sp, sp, 16
	sub sp, sp, #16
	str w0, [sp, 12]
	
	mov w0, #1
	ldr w1, [sp, 12]
	mul w0, w1, w0
	add sp, sp, 16
	ldr w1, [sp, 12]
	add w0, w0, w1
	add sp, sp, 16
	ldr w1, [sp, 16]
	cmp w1, w0
	ble .L8
	mov w0, 0
	b .L9
.L8:
	mov w0, 1
.L9:
	add sp, sp, 16
	sub sp, sp, #16
	str w0, [sp, 12]
	
	mov w0, #0
	ldr w1, [sp, 12]
	cmp w0, w1
	cset w0, ne
	add sp, sp, 16
	sub sp, sp, #16
	str w0, [sp, 12]
	
	mov w0, #1
	ldr w1, [sp, 12]
	cmp w1, w0
	cset w0, eq
	add sp, sp, 16
	sub sp, sp, #16
	str w0, [sp, 12]
	
	mov w0, #2
	ldr w1, [sp, 12]
	cmp w1, 0
	beq .L10
	cmp w0, 0
	beq .L10
	mov w0, 1
	b .L11
.L10:
	mov w0, 0
.L11:
	add sp, sp, 16
	sub sp, sp, #16
	str w0, [sp, 12]
	
	mov w0, #3
	ldr w1, [sp, 12]
	cmp w1, 0
	bne .L12
	cmp w0, 0
	beq .L13
.L12:
	mov w0, 1
	b .L14
.L13:
	mov w0, 0
.L14:
	add sp, sp, 16
	ret
