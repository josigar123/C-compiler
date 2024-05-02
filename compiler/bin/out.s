	.text
	.global _main

_main:
	
                        
	mov w0, #155
                        
	sub sp, sp, 16
                        
	str x0, [sp, 12]
                        
	
	mov w0, #800
	sub sp, sp, #16
                            
	str x0, [sp, 12]
	mov w0, #1
                            
	ldr x1, [sp, 12]
                            
	add x0, x0,x1
                            
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
                            
	sub sp, sp, #16
                            
	str x0, [sp, 12]
                            
	
	mov w0, #3
	sub sp, sp, #16
                            
	str x0, [sp, 12]
	mov w0, #5
                            
	ldr x1, [sp, 12]
                            
	add x0, x0,x1
                            
	add sp, sp, 16
                            
	sub sp, sp, #16
                            
	str x0, [sp, 12]
                            
	
                        
	mov w0, #10
                        
	sub sp, sp, 16
                        
	str x0, [sp, 12]
                        
	
	mov w0, #20
                        
	ldr x1, [sp, 12]
                        
	cmp x1, x0
                        
	blt .L4
                        
	mov w0, 0
                        
	 b .L5
                        
.L4:
                        
	mov w0, 1
                        
.L5:
                        
	add sp, sp, 16
                            
	ldr x1, [sp, 12]
                            
	cmp x1, 0
                            
	beq .L6
                            
	cmp x0, 0
                            
	beq .L6
                            
	mov x0, 1
                            
	b   .L7
                            
.L6:
                            
	mov w0, 0
                            
.L7:
                            
	add sp, sp, 16
                            
	ldr x1, [sp, 12]
                            
	cmp x1, 0
                            
	bne .L8 
                            
	cmp x0, 0
                            
	beq .L9
                            
.L8:
                            
	mov x0, 1
                            
	b   .L10
                            
.L9:
                            
	mov x0, 0
                            
.L10:
                            
	add sp, sp, 16
	ret
