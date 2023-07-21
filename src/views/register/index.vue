<template>
  <div class="container">
    <el-button class="btn-back" type="primary" round @click="goBack">返回上一页</el-button>

    <el-form class="register-form" ref="registerFormRef" :model="registerForm" :rules="rules" label-width="120px"
      status-icon>
      <el-form-item label="注册用户" prop="user_name">
        <el-input v-model="registerForm.user_name" placeholder="请输入用户名" clearable>
        </el-input>
      </el-form-item>

      <el-form-item label="注册密码" prop="user_pwd">
        <el-input v-model="registerForm.user_pwd" placeholder="请输入密码" show-password clearable>
        </el-input>
      </el-form-item>

      <el-form-item>
        <el-button class="btn-register" type="primary" @click="handleRegister">注册</el-button>
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

const registerFormRef = ref<any>();

const registerForm = reactive({
  user_name: "",
  user_pwd: "",
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

const goBack = () => {
  router.back();
};

const handleRegister = async () => {
  if (!registerFormRef.value) return;
  await registerFormRef.value.validate(async (valid: boolean) => {
    if (valid) {
      console.log("submit!");
      const res = await UserCollection.getInstance().addUser({
        ...registerForm
      })
      console.log(res);
      ElMessage({
        message: "注册用户成功",
        type: "success",
      });
      goBack();
    } else {
      console.log("error submit!");
    }
  });
};
</script>

<style lang="scss" scoped>
@import "./index.scss";
</style>
