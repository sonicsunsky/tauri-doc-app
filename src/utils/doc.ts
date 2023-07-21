import { invoke } from "@tauri-apps/api/tauri";

interface Doc {
    id?: string,
    name: string,
    sex: string,
    birthday: string,
    candidate_type: string,
    identity_num: string,
    company_name: string,
    company_type: string,
    contract_time: string,
    create_time?: string
}

class DocCollection {
    static instance: DocCollection;

    public static getInstance(): DocCollection {
        if (!DocCollection.instance) {
            DocCollection.instance = new DocCollection();
        }
        return DocCollection.instance;
    }

    async getAllDocs(): Promise<Doc[]> {
        const docList: Doc[] = await invoke("get_all_docs");
        // 返回获取到的会话
        return docList;
    }

    async getDocsByStmt(stmt: string): Promise<Doc[]> {
        const docList: Doc[] = await invoke("get_docs_by_stmt", { stmt });
        // 返回获取到的会话
        return docList;
    }

    async addDoc(doc: Doc): Promise<Doc> {
        const result: Doc = await invoke("add_doc", { doc });
        return result;
    }
}

export type { Doc };
export { DocCollection }; // 导出接口和类，以便在其他文件中使用
