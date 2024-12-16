interface SqlUser {
    username: string,
    is_admin: boolean,
    created_at: string,
}


interface SqlUserProps {
    username: string;
    password: string;
    is_admin: boolean;
}

type SqlUserList = SqlUser[];
export {
    type SqlUser,
    type SqlUserProps,
    type SqlUserList,
}