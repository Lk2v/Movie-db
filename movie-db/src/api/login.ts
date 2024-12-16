import { invoke } from "@tauri-apps/api/core";
import type { SqlUser } from "./sql_user";

interface SqlUserCredentials {
    username: string;
    password: string;
}

interface SqlUserChoice {
    title?: string;
    username?: string;
}

async function login(user: SqlUserCredentials) {
    return invoke("login_user", { user });
}

async function logout() {
    return invoke("logout_user");
}

async function getLoggedUsername() {
    return invoke("get_logged_username");
}

async function getCurrentUser() {
    return await invoke<SqlUser>("get_sql_users");
}

export { 
    login, 
    logout,
    
    getLoggedUsername,
    getCurrentUser,
    type SqlUserCredentials, 
    type SqlUserChoice
};

