import { useTurso } from "@/server/utils/turso";

export default defineEventHandler(async (event) => {
    const client = useTurso(/* event */);
    const { rows } = await client.execute("select * from states");

    // console.log("Rows:", rows);
    return {
        data: {
            items: rows,
        },
    };
});
