	.text
	.global _main

_main:
	
	sub sp, sp, #16
                    
	
	mov w0, #10
                    
	
                    str x0, [sp,12]
                    
	mov x0, 0
                    
	add sp, sp, 16
	
	
	mov w0, #1
	sub sp, sp, #16
	str x0, [sp, 12]

	mov w0, #1
	ldr x1, [sp, 12]
	sub x0, x1, x0
	add sp, sp, 16
	sub sp, sp, 16
	str x0, [sp, 12]
	
	mov w0, #1
	neg x0, x0
	sub sp, sp, #16
	str x0, [sp, 12]
	
	mov w0, #100
	mvn x0, x0
	sub sp, sp, #16
	str x0, [sp, 12]
	mov w0, #6
	cmp x0, #0
                        
	mov x0, #0
                        
	cset x0, eq
	ldr x1, [sp, 12]
	add x0, x0, x1
	add sp, sp, 16
	ldr x1, [sp, 12]
	mul x0, x1, x0
	add sp, sp, 16
	ldr x1, [sp, 12]
	cmp x1, x0
	bgt .L2
	mov w0, 0
	 b .L3
.L2:
	mov w0, 1
.L3:
	add sp, sp, 16
	ret
