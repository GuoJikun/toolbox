export const type = (val: any) => {
    return Object.prototype.toString.call(val).slice(8, -1).toLowerCase()
}

export const isObject = (val: any) => {
    return type(val) === 'object'
}

export const isArray = (val: any) => {
    return type(val) === 'array'
}
