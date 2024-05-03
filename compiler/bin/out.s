	.text
	.global _main

_main:
	
                        
                        
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
                        
	sub sp, sp, 16
                        
	str x0, [sp, 12]
                        
	
	mov w0, #3
                        
	ldr x1, [sp, 12]
                        
	cmp x1, x0
                        
	blt .L4
                        
	mov w0, 0
                        
	 b .L5
                        
.L4:
                        
	mov w0, 1
                        
.L5:
                        
	add sp, sp, 16
	ret
