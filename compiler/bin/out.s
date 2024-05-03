	.text
	.global _main

_main:
	
	mov w0, #1
	sub sp, sp, #16
                            
	str x0, [sp, 12]
	mov w0, #2
	sub sp, sp, #16
                            
	str x0, [sp, 12]
	
	mov w0, #2
                            
	ldr x1, [sp, 12]
                            
	mul x0, x1, x0
                            
	add sp, sp, 16
                            
	ldr x1, [sp, 12]
                            
	add x0, x0,x1
                            
	add sp, sp, 16
                            
	sub sp, sp, #16
                            
	str x0, [sp, 12]
                            
	
	mov w0, #3
	sub sp, sp, #16
                            
	str x0, [sp, 12]
	mov w0, #3
	mvn x0, x0
                            
	ldr x1, [sp, 12]
                            
	add x0, x0,x1
                            
	add sp, sp, 16
	sub sp, sp, #16
                            
	str x0, [sp, 12]
                            
	mov w0, #1
	sub sp, sp, #16
	str x0, [sp, 12]
	
	mov w0, #3
	neg x0, x0
	ldr x1, [sp, 12]
	sdiv x0, x1, x0
	add sp, sp, 16
	ldr x1, [sp, 12]
                            
	sub x0, x1, x0
                            
	add sp, sp, 16
                            
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
                            
	sub sp, sp, #16
                            
	str x0, [sp, 12]
                            
	
                        
                        
	mov w0, #2
                        
	sub sp, sp, #16
                        
	str x0, [sp, 12]
                        
	
	mov w0, #1
                        
	ldr x1, [sp, 12]
                        
	cmp x0, x1
                        
	cset x0, ne
                        
	add sp, sp , 16
                        
	sub sp, sp, #16
                        
	str x0, [sp, 12]
                        
	
                        
	mov w0, #0
                        
	sub sp, sp, #16
                        
	str x0, [sp, 16]
                        
	
	mov w0, #2
                        
	ldr x1, [sp, 16]
                        
	cmp x1, x0
                        
	ble .L4
                        
	mov x0, 0
                        
	b   .L5
                        
.L4:
                        
	mov x0, 1
                        
.L5:
                        
	add sp, sp, 16
                        
	ldr x1, [sp, 12]
                        
	cmp x1, x0
                        
	cset x0, eq
                        
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
                            
	sub sp, sp, #16
                            
	str x0, [sp, 12]
                            
	
                        
	mov w0, #2
                        
	sub sp, sp, #16
                        
	str x0, [sp, 16]
                        
	
	mov w0, #1
                        
	ldr x1, [sp, 16]
                        
	cmp x1, x0
                        
	bge .L9
                        
	mov x0, 0
                        
	b   .L10
                        
.L9:
                        
	mov x0, 1
                        
.L10:
                        
	add sp, sp, 16
                            
	ldr x1, [sp, 12]
                            
	cmp x1, 0
                            
	bne .L11 
                            
	cmp x0, 0
                            
	beq .L12
                            
.L11:
                            
	mov x0, 1
                            
	b   .L13
                            
.L12:
                            
	mov x0, 0
                            
.L13:
                            
	add sp, sp, 16
	ret
