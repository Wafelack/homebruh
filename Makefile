SC=csc
BINARY=bruh

$(BINARY) : *.o
	$(SC) $< -o $@

%.o : %.scm
	$(SC) -c $<

.PHONY : clear
clean :
	@rm *.o
	@rm $(BINARY)
