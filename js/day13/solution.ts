function fold(c: Coord, f: Coord): Coord {
    if(c.x >= f.x && c.y >= f.y){
        if(f.x > 0){
            return {
                x: c.x - 2*(c.x-f.x),
                y: c.y
            }
        }
        if(f.y > 0){
            return {
                x: c.x, 
                y: c.y - 2*(c.y-f.y)
            }
        }
    }
    return c
}

interface CoordsMap {
    [index: string]: Coord
}

function printCoords(coords: CoordsMap) {
    const max = {x: 0, y: 0};
    for(const i in coords){
        const c = coords[i];
        if(c.x > max.x) {
            max.x = c.x;
        }
        if(c.y > max.y){
            max.y = c.y;
        }
    }
    
    let out = "";
    for(let j=0; j<=max.y; j++){
        for(let i=0; i<=max.x; i++) {
            const c = {x: i, y: j};
            if(coords[c.x + ',' + c.y] === undefined){
                out += " "
            } else {
                out += "▓"
            }
        }
        out += "\n"
    }
    console.log(out);
}

interface Coord {
    x: number,
    y: number
}

interface CoordFolds {
    coords: Coord[],
    folds: Coord[]
}

function getCoordsAndFolds(input: string) : CoordFolds {
    const split = input.split('\n\n');
    const coords: Coord[] = split[0]
        .split('\n')
        .map((x)=>x.split(',').map((x)=>Number.parseInt(x)))
        .map((c)=>{ return {x: c[0], y: c[1]} })
    const folds = 
        split[1].split('\n').map((x)=>x.replace('fold along ', '').split('='))
        .map((f)=> {
        if(f[0] == 'x'){
            return {x: Number.parseInt(f[1]), y: 0}
        } else {
            return {x: 0, y: Number.parseInt(f[1])}
        }
    })

    return {
        coords: coords,
        folds: folds,
    }
}

function partOne(input: string): number {
    const cf = getCoordsAndFolds(input);
    const f = cf.folds[0];

    const coordsMap : CoordsMap = {};

    cf.coords
        .map((c)=> fold(c, f))
        .forEach((c)=>{coordsMap[c.x +',' + c.y]=c})

    return Object.keys(coordsMap).length;
}

function partTwo(input: string): number {
    const cf = getCoordsAndFolds(input);

    let coordsMap : CoordsMap = {};
    let coords = cf.coords;

    for(const f of cf.folds){ 
        coordsMap = {}
        coords.map((c)=> fold(c, f)).forEach((c)=>{coordsMap[c.x +',' + c.y]=c})
        coords = []
        for(const v in coordsMap){
            coords.push(coordsMap[v])
        }
    }
    printCoords(coordsMap);
    return -1;
}

export { partOne, partTwo };