from PIL import Image, ImageDraw, ImageFont
# Исходные данные
Place_1 = [[0,  0, 0, 0, 'x',  0, 0, 0,  0,  0,  0,  0, 0,  0, 0, 0],
        [0,  0, 0, 0, 'x',  0, 0, 0,  0,  0,  0,  0, 0, 'b', 0, 0],
        [0,  0, 0, 0, 'x',  0, 0, 0,  0,  0,  0,  0, 0,  0, 0, 0],
        [0,  'a', 0, 0, 'x',  0, 0, 0,  0,  0,  0, 'x', 0,  0, 0, 0],
        [0,  0, 0, 0, 'x',  0, 0, 0,  0, 'x', 'x', 'x', 0,  0, 0, 0],
        [0,  0, 0, 0, 'x',  0, 0, 0,  0, 'x', 'x', 'x', 0,  0, 0, 0],
        [0,  0, 0, 0, 'x',  0, 0, 0, 'x', 'x', 'x', 'x', 0,  0, 0, 0],
        [0,  0, 0, 0, 'x',  0, 0, 0, 'x', 'x', 'x', 'x', 0,  0, 0, 0],
        [0,  0, 0, 0, 'x',  0, 0, 0,  0, 'x', 'x', 'x', 0,  0, 0, 0],
        [0,  0, 0, 0, 'x',  0, 0, 0,  0,  0, 'x', 'x', 0,  0, 0, 0],
        [0,  0, 0, 0,  0, 'x', 0, 0,  0,  0, 'x', 'x', 0,  0, 0, 0],
        [0,  0, 0, 0,  0, 'x', 0, 0,  0,  0, 'x', 'x', 0,  0, 0, 0],
        [0,  0, 0, 0,  0,  0, 0, 0,  0,  0, 'x', 'x', 0,  0, 0, 0],
        [0,  0, 0, 0,  0,  0, 0, 0,  0,  0,  0, 'x', 0,  0, 0, 0],
        [0,  0, 0, 0,  0,  0, 0, 0,  0,  0,  0, 'x', 0,  0, 0, 0],
        [0,  0, 0, 0,  0,  0, 0, 0,  0,  0,  0,  0, 0,  0, 0, 0]]

def main():
    
    PL = Image.new('RGB', (1600, 1600), (255, 255, 255))
    re_PL = ImageDraw.Draw(PL)
    front = ImageFont.truetype("arial.ttf", 50)
    
    ShowPlace(Place_1)
    PL.show()

# Функция разметки пространства
def ShowPlace(Place):
    n = 0
    for i in Place:
        n += 1
        m = 0
        for j in i:
            m += 1
            if j == 'x':
                re_PL.rectangle((m*100-100, n*100-100, m*100, n*100),
                                fill='black', outline=(255, 255, 255))
            elif j == 'a' or j == 'b':
                re_PL.rectangle((m*100-100, n*100-100, m*100, n*100),
                                fill='yellow', outline=(0, 0, 0))
                re_PL.text((m*100-80, n*100-80), j, fill='black', font=front)
            else:
                re_PL.rectangle((m*100-100, n*100-100, m*100, n*100),
                                fill='white', outline=(0, 0, 0))
                re_PL.text((m*100-80, n*100-80), str(j),
                           fill='black', font=front)
                
if __name__ == '__main__':
    main()