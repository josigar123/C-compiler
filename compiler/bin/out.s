	.text
	.global _main

_main:
	
	sub sp, sp, #16
                    
	
	mov w0, #1
                    
	
                    str x0, [sp,12]
                    
	mov x0, 0
                    
	add sp, sp, 16
	
	
	sub sp, sp, #16
                    
	
	mov w0, #2
                    
	
                    str x0, [sp,12]
                    
	mov x0, 0
                    
	add sp, sp, 16
	
	
	sub sp, sp, #16
                    
	
	mov w0, #3
                    
	
                    str x0, [sp,12]
                    
	mov x0, 0
                    
	add sp, sp, 16
	
	Unsupported statement
	
	mov w0, #1
	ret
