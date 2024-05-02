	.text
	.global _main

_main:
	
	mov w0, #1
                            
	sub sp, sp, #16
                            
	str x0, [sp, 12]
                            
	
	mov w0, #0
	sub sp, sp, #16
                            
	str x0, [sp, 12]
	mov w0, #1
	sub sp, sp, #16
                            
	str x0, [sp, 12]
                            
	mov w0, #1
	ldr x1, [sp, 12]
                            
	sub x0, x1, x0
                            
	add sp, sp, 16
                            
	sub sp, sp, #16
                            
	str x0, [sp, 12]
                            
	
	mov w0, #2
                            
	ldr x1, [sp, 12]
                            
	cmp x1, 0
                            
	beq .L2
                            
	cmp x0, 0
                            
	beq .L2
                            
	mov x0, 1
                            
	b   .L3
                            
.L2:
                            
	mov w0, 0
                            
.L3:
                            
	add sp, sp, 16
                            
	ldr x1, [sp, 12]
                            
	add x0, x0,x1
                            
	add sp, sp, 16
                            
	ldr x1, [sp, 12]
                            
	cmp x1, 0
                            
	bne .L4 
                            
	cmp x0, 0
                            
	beq .L5
                            
.L4:
                            
	mov x0, 1
                            
	b   .L6
                            
.L5:
                            
	mov x0, 0
                            
.L6:
                            
	add sp, sp, 16
	ret
