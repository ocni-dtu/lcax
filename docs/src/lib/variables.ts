export const baseUrl = `${process.env.DOMAIN?.startsWith('localhost') ? 'http://' : 'https://'}${process.env.DOMAIN}`
