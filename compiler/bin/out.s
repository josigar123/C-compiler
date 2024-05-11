	.text
	.global _main

_main:
	
	sub sp, sp, #16
                    
	
	mov w0, #3
                    
	
                    str x0, [sp,12]
                    
	mov x0, 0
                    
	add sp, sp, 16
	
	
	mov w0, #1
	sub sp, sp, 16
	str x0, [sp, 12]
	
	mov w0, #2
	ldr x1, [sp, 12]
	cmp x1, x0
	blt .L2
	mov w0, 0
	b .L3
.L2:
	mov w0, 1
.L3:
	add sp, sp, 16
	
	sub sp, sp, #16
	str x0, [sp, 12]
	
	mov w0, #3
	sub sp, sp, #16
	str x0, [sp, 12]
	
	mov w0, #1
	ldr x1, [sp, 12]
	cmp x1, 0
	beq .L4
	cmp x0, 0
	beq .L4
	mov x0, 1
	b   .L5
.L4:
	mov w0, 0
.L5:
	add sp, sp, 16
	ldr x1, [sp, 12]
	cmp x1, 0
	bne .L6
	cmp x0, 0
	beq .L7
.L6:
	mov x0, 1
	b   .L8
.L7:
	mov x0, 0
.L8:
	add sp, sp, 16
	ret
