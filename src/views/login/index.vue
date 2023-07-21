<template>
  <div class="container">
    <el-form class="login-form" ref="loginFormRef" :model="loginForm" :rules="rules" label-width="120px" status-icon>
      <el-form-item label="登录用户" prop="user_name">
        <el-input v-model="loginForm.user_name" placeholder="请输入用户名" clearable>
        </el-input>
      </el-form-item>

      <el-form-item label="登录密码" prop="user_pwd">
        <el-input v-model="loginForm.user_pwd" placeholder="请输入密码" show-password clearable>
        </el-input>
      </el-form-item>

      <el-form-item>
        <el-button class="btn-login" type="primary" @click="handleLogin">登录</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from "vue";
import { ElMessage } from "element-plus";
import { useRouter } from "vue-router";

import { UserCollection } from "@/utils/user";
import type { User } from "@/utils/user";

const router = useRouter();

const loginFormRef = ref<any>();

const loginForm = reactive({
  user_name: "admin",
  user_pwd: "123456",
});

const rules = reactive({
  user_name: [{ required: true, message: "请输入用户名", trigger: "blur" }],
  user_pwd: [
    {
      required: true,
      message: "请输入密码",
      trigger: "blur",
    },
  ],
});

const checkLoginUser = async () => {
  if (loginForm.user_name === "admin" && loginForm.user_pwd === "123456") {
    router.push("/home");
  } else {
    const condition = `user_name like '%${loginForm.user_name}%'`
    const res: User[] = await UserCollection.getInstance().getUserByName(condition)
    console.log("checkLoginUser: ", res);
    if (!res) {
      ElMessage({
        message: "用户未注册",
        type: "error",
      });
      return;
    }
    if (res[0].user_name === loginForm.user_name && res[0].user_pwd === loginForm.user_pwd) {
      router.push("/home");
    }
  }
};

const handleLogin = async () => {
  if (!loginFormRef.value) return;
  await loginFormRef.value.validate((valid: boolean) => {
    if (valid) {
      console.log("submit!");
      localStorage.setItem("login", JSON.stringify(loginForm));
      checkLoginUser();
    } else {
      console.log("error submit!");
    }
  });
};
</script>

<style lang="scss" scoped>
@import "./index.scss";
</style>
