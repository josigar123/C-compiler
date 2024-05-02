	.text
	.global _main

_main:
	
	mov w0, #0
                            
	sub sp, sp, #16
                            
	str x0, [sp, 12]
                            
	
	mov w0, #0
                            
	ldr x1, [sp, 12]
                            
	cmp x1, 0
                            
	bne .L2
                            
	cmp x0, 0
                            
	beq .L3
                            
.L2:
                            
	mov x0, 1
                            
	b   .L4
                            
.L3:
                            
	mov x0, 0
                            
.L4:
                            
	add sp, sp, 16
	ret
