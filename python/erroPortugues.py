entrada = input()
void = ''
upper = True

for letra in entrada:

    if letra == "." :
        void += letra
        upper = True

    elif letra != "." and letra != '' and upper:

        void += letra.upper()
        upper = False

    else:
        void += letra

print(void)
 