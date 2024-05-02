	.text
	.global _main

_main:
	
                        
	mov w0, #2
                        
	sub sp, sp, #16
                        
	str x0, [sp, 16]
                        
	
	mov w0, #1
                        
	ldr x1, [sp, 16]
                        
	cmp x1, x0
                        
	ble .L2
                        
	mov x0, 0
                        
	b   .L3
                        
.L2:
                        
	mov x0, 1
                        
.L3:
                        
	add sp, sp, 16
	ret
