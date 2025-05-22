def find_pos(grid, symbol):
    for y in range(len(grid)):
        for x in range(len(grid[0])):
            if grid[y][x] == symbol:
                return (x, y)
    return None

def is_walkable(grid, x, y):
    if 0 <= x < len(grid[0]) and 0 <= y < len(grid):
        return grid[y][x] != 'x'
    return False

def direction(x1, y1, x2, y2):
    dx, dy = x2 - x1, y2 - y1
    if dx != 0: dx = dx // abs(dx)
    if dy != 0: dy = dy // abs(dy)
    return (dx, dy)

def get_successors(grid, node, goal):
    successors = []
    x, y = node
    
    # Получаем все возможные направления движения
    directions = [(dx, dy) for dx in [-1, 0, 1] for dy in [-1, 0, 1] if not (dx == 0 and dy == 0)]
    
    for dx, dy in directions:
        # Пропускаем непроходимые клетки
        if not is_walkable(grid, x + dx, y + dy):
            continue
            
        # Если двигаемся по диагонали, проверяем ортогональные направления
        if dx != 0 and dy != 0:
            if not is_walkable(grid, x + dx, y) and not is_walkable(grid, x, y + dy):
                continue
                
        # Выполняем прыжок в этом направлении
        jump_point = jump(grid, x, y, dx, dy, goal)
        if jump_point:
            successors.append(jump_point)
    
    return successors

def jump(grid, x, y, dx, dy, goal):
    nx, ny = x + dx, y + dy
    
    # Проверяем базовые условия
    if not is_walkable(grid, nx, ny):
        return None
        
    # Если достигли цели
    if (nx, ny) == goal:
        return (nx, ny)
        
    # Проверяем принудительных соседей
    if has_forced_neighbors(grid, nx, ny, dx, dy):
        return (nx, ny)
        
    # Для диагонального движения проверяем ортогональные направления
    if dx != 0 and dy != 0:
        if jump(grid, nx, ny, dx, 0, goal) or jump(grid, nx, ny, 0, dy, goal):
            return (nx, ny)
    
    # Продолжаем прыжок в том же направлении
    return jump(grid, nx, ny, dx, dy, goal)

def has_forced_neighbors(grid, x, y, dx, dy):
    if dx != 0 and dy != 0:  # Диагональное движение
        # Проверяем горизонтальное направление
        if not is_walkable(grid, x - dx, y) and is_walkable(grid, x - dx, y + dy):
            return True
        # Проверяем вертикальное направление
        if not is_walkable(grid, x, y - dy) and is_walkable(grid, x + dx, y - dy):
            return True
    else:  # Ортогональное движение
        if dx != 0:  # Горизонтальное
            if not is_walkable(grid, x, y + 1) and is_walkable(grid, x + dx, y + 1):
                return True
            if not is_walkable(grid, x, y - 1) and is_walkable(grid, x + dx, y - 1):
                return True
        else:  # Вертикальное
            if not is_walkable(grid, x + 1, y) and is_walkable(grid, x + 1, y + dy):
                return True
            if not is_walkable(grid, x - 1, y) and is_walkable(grid, x - 1, y + dy):
                return True
    return False

def reconstruct_path(came_from, current):
    path = [current]
    while current in came_from:
        current = came_from[current]
        path.append(current)
    path.reverse()
    return path

def jps(grid, start, goal):
    open_set = [start]
    came_from = {}
    g_score = {start: 0}
    f_score = {start: heuristic(start, goal)}
    
    while open_set:
        current = min(open_set, key=lambda x: f_score.get(x, float('inf')))
        
        if current == goal:
            return reconstruct_path(came_from, current)
            
        open_set.remove(current)
        
        for neighbor in get_successors(grid, current, goal):
            tentative_g = g_score[current] + distance(current, neighbor)
            
            if neighbor not in g_score or tentative_g < g_score[neighbor]:
                came_from[neighbor] = current
                g_score[neighbor] = tentative_g
                f_score[neighbor] = g_score[neighbor] + heuristic(neighbor, goal)
                if neighbor not in open_set:
                    open_set.append(neighbor)
    
    return None  # Путь не найден

def heuristic(a, b):
    # Манхэттенское расстояние
    return abs(a[0] - b[0]) + abs(a[1] - b[1])

def distance(a, b):
    dx = abs(a[0] - b[0])
    dy = abs(a[1] - b[1])
    return dx + dy if (dx == 0 or dy == 0) else 1.414  # sqrt(2)

# Тестовая сетка
grid = [
    [0,  0, 0, 0, 'x',  0, 0, 0,  0,  0,  0,  0, 0,  0, 0, 0],
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
    [0,  0, 0, 0,  0,  0, 0, 0,  0,  0,  0,  0, 0,  0, 0, 0]
]

start = find_pos(grid, 'a')
goal = find_pos(grid, 'b')

if start and goal:
    path = jps(grid, start, goal)
    if path:
        print("Найден корректный путь:")
        print(path)
        
        # Визуализация
        visual_grid = [row.copy() for row in grid]
        for x, y in path[1:-1]:  # Не отмечаем старт и финиш
            visual_grid[y][x] = '*'
        
        print("\nВизуализация пути:")
        for row in visual_grid:
            print(' '.join(str(cell) for cell in row))
    else:
        print("Путь не найден!")
else:
    print("Старт или цель не найдены в сетке!")