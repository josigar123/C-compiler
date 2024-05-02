	.text
	.global _main

_main:
	
                        
	mov w0, #2
	sub sp, sp, #16
                            
	str x0, [sp, 12]
	
	mov w0, #10
                            
	ldr x1, [sp, 12]
                            
	mul x0, x1, x0
                            
	add sp, sp, 16
                        
	sub sp, sp, #16
                        
	str x0, [sp, 16]
                        
	
	mov w0, #2
	sub sp, sp, #16
                            
	str x0, [sp, 12]
	mov w0, #1
                            
	ldr x1, [sp, 12]
                            
	add x0, x0,x1
                            
	add sp, sp, 16
                        
	ldr x1, [sp, 16]
                        
	cmp x1, x0
                        
	bge .L2
                        
	mov x0, 0
                        
	b   .L3
                        
.L2:
                        
	mov x0, 1
                        
.L3:
                        
	add sp, sp, 16
	ret
