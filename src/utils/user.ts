import { invoke } from "@tauri-apps/api/tauri";

interface User {
    id?: string;
    user_name: string;
    user_pwd: string;
    create_time?: string;
}

class UserCollection {
    static instance: UserCollection;
    public static getInstance(): UserCollection {
        if (!UserCollection.instance) {
            UserCollection.instance = new UserCollection();
        }
        return UserCollection.instance;
    }

    async getAllUsers(): Promise<User[]> {
        const userList: User[] = await invoke("get_all_users");
        return userList;
    }

    async getUserByName(name: string): Promise<User[]> {
        const user: User[] = await invoke("get_user_by_name", { name });
        return user;
    }

    async addUser(user: User): Promise<User> {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        const result: User = await invoke("add_user", { user });
        return result;
    }

    async deleteUser(id: string): Promise<void> {
        await invoke("delete_user", {
            id,
        });
    }

    async updateUser(id: string, name: string): Promise<void> {
        await invoke("update_user", {
            id, //会自动转换为整形
            name,
        });
    }
}

export type { User };
export { UserCollection }; // 导出接口和类，以便在其他文件中使用
