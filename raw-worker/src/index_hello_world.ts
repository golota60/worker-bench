export default {
    async fetch(req: Request, env: any) { // Request that user made
        return new Response("Hello, World!")
    }
}