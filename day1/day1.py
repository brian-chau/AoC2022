import sys

class Elf:
    def __init__( self ):
        self.points = 0

def read_file( filename: str ) -> list:
    with open( filename ) as f:
        lines = f.readlines()
    return lines

def determine_points( content: list, count: int ):
    elves = []
    elf = Elf()
    for line in content:
        if line.strip() == '':
            elves.append( elf )
            elf = Elf()
        else:
            elf.points += int(line)

    top_points = sum( elf.points for elf in sorted(elves, key=lambda x: x.points, reverse=True )[:count] )
    print( f'Max: {top_points}' )

if __name__ == '__main__':
    content = read_file( 'input.txt' )
    determine_points( content, 1 )
    determine_points( content, 3 )