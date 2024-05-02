	.text
	.global _main

_main:
	
                        
	mov w0, #1
                        
	sub sp, sp, 16
                        
	str x0, [sp, 12]
                        
	
	mov w0, #2
                        
	ldr x1, [sp, 12]
                        
	cmp x1, x0
                        
	cset x0, eq
                        
	add sp, sp, 16
	ret
