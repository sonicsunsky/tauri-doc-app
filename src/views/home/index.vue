<template>
  <div class="page">
    <div class="title">资料管理应用</div>

    <el-form ref="formRef" :model="formData" :rules="rules" status-icon>
      <el-form-item label="姓名" prop="name">
        <el-input v-model="formData.name" placeholder="请填写姓名" />
      </el-form-item>

      <el-form-item label="性别" prop="sex">
        <el-select v-model="formData.sex" placeholder="请选择性别">
          <el-option label="男" value="男" />
          <el-option label="女" value="女" />
        </el-select>
      </el-form-item>

      <el-form-item label="出生年月" prop="birthday">
        <el-date-picker v-model="formData.birthday" type="date" value-format="YYYY-MM-DD" placeholder="请选择出生日期" />
      </el-form-item>

      <el-form-item label="考生类别" prop="candidate_type">
        <el-radio-group v-model="formData.candidate_type">
          <el-radio label="在职人员" />
          <el-radio label="已签订就业协议的应届毕业生" />
        </el-radio-group>
      </el-form-item>

      <el-form-item label="身份证号" prop="identity_num">
        <el-input v-model="formData.identity_num" placeholder="请填写身份证号" />
      </el-form-item>

      <el-form-item label="单位名称" prop="company_name">
        <el-input v-model="formData.company_name" placeholder="请填写单位名称" />
      </el-form-item>

      <el-form-item label="公司性质" prop="company_type">
        <el-input v-model="formData.company_type" placeholder="请填写公司性质" />
      </el-form-item>

      <el-form-item label="签约时间" prop="contract_time">
        <el-date-picker v-model="formData.contract_time" type="date" value-format="YYYY-MM-DD" placeholder="请选择签约时间" />
      </el-form-item>

      <el-form-item>
        <el-button type="primary" @click="exportDoc">导出文档</el-button>
        <el-button type="primary" @click="openFileList">查看文档</el-button>
        <el-button @click="goLogin">去登录</el-button>
        <el-button v-if="isAdmin" @click="goRegister">去注册</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from "vue";
import type { FormRules } from "element-plus";
import { genFileId } from "element-plus";

import docxtemplater from "docxtemplater";
import JSZipUtils from "jszip-utils";
import { saveAs } from "file-saver";
import PizZip from "pizzip";

import { useRouter } from "vue-router";

import { DocCollection } from "@/utils/doc";
import { save, DialogFilter, message } from '@tauri-apps/api/dialog';
import { downloadDir } from '@tauri-apps/api/path';
import { writeBinaryFile } from '@tauri-apps/api/fs';

const router = useRouter();

const formRef = ref<any>();

const filters: DialogFilter[] = [
  { name: "Word 97-2004", extensions: ["doc"] },
  { name: "Word 2007", extensions: ["docx"] }
];

const formData = reactive({
  name: "张三",
  sex: "男",
  birthday: "1989-10-10",
  candidate_type: "在职人员",
  is_staff: false,
  is_graduate: false,
  identity_num: "370213198007067689",
  company_name: "淘宝",
  company_type: "上市公司",
  contract_time: "2022-03-01",
});

formData.is_staff = computed(() => formData.candidate_type === "在职人员").value;
formData.is_graduate = computed(() => formData.candidate_type !== "在职人员").value;

const rules: FormRules = reactive({
  name: [{ required: true, message: "姓名不能为空", trigger: "blur" }],
  sex: [
    {
      required: true,
      message: "性别不能为空",
      trigger: "change",
    },
  ],
  birthday: [
    {
      type: "date",
      required: true,
      message: "请选择出生日期",
      trigger: "change",
    },
  ],
  identity_num: [
    {
      required: true,
      message: "请输入身份证号",
      trigger: "trigger",
    },
  ],
  candidate_type: [
    {
      required: true,
      message: "请选择考生类别",
      trigger: "change",
    },
  ],
  company_name: [
    {
      required: true,
      message: "请输入单位名称",
      trigger: "trigger",
    },
  ],
  company_type: [
    {
      required: true,
      message: "请输入单位名称",
      trigger: "trigger",
    },
  ],
  contract_time: [
    {
      type: "date",
      required: true,
      message: "请选择签约时间",
      trigger: "change",
    },
  ],
});

const isAdmin = computed(
  () => {
    const loginRes = localStorage.getItem("login")
    if (loginRes) {
      return JSON.parse(loginRes).user_name === "admin"
    } else return false
  }
);

onMounted(() => {
});

const goRegister = () => {
  router.push("/register");
};

const goLogin = () => {
  router.push("/");
};

const openFileList = () => {
  router.push("/file");
};

const exportWord = () => {
  // 读取并获得模板文件的二进制内容
  JSZipUtils.getBinaryContent("test.docx", async (error: any, content: any) => {
    if (error) throw error; // 抛出异常
    // 创建一个PizZip实例，内容为模板的内容
    const zip = new PizZip(content);
    const doc = new docxtemplater().loadZip(zip); // 创建并加载docxtemplater实例对象
    // 去除未定义值所显示的undefined
    doc.setOptions({
      nullGetter: function () {
        return "";
      },
    });
    doc.setData({ ...formData }); // 设置模板变量的值
    try {
      doc.render(); // 用模板变量的值替换所有模板变量
    } catch (error: any) {
      const e = {
        message: error.message,
        name: error.name,
        stack: error.stack,
        properties: error.properties,
      };
      console.log(JSON.stringify({ error: e }));
      throw error; // 抛出异常
    }

    // 生成一个代表docxtemplater对象的zip文件（不是一个真实的文件，而是在内存中的表示）
    let out = doc.getZip().generate({
      type: "blob",
      mimeType:
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    });

    const loginRes = localStorage.getItem("login")
    const userName = loginRes ? JSON.parse(loginRes).user_name : "";
    const fileName = `导出文档-${userName}.docx`
    // 将目标文件对象保存为目标类型的文件，并命名
    // downloadFile(out, fileName)
    // saveAs(out, fileName);
    handleSaveFile(out, fileName)
  });
};

const downloadFile = (blob: Blob, fileName: string) => {
  // 存在浏览器兼容性
  let href = URL.createObjectURL(new Blob([blob]));
  let alink = document.createElement('a');
  alink.style.display = 'none';
  alink.download = fileName; //下载后文件名
  alink.href = href;
  document.body.appendChild(alink);
  alink.click();
  console.log("alink: ", alink);
  document.body.removeChild(alink); //下载完成移除元素
  URL.revokeObjectURL(href); //释放掉blob对象
}

const handleSaveFile = async (blob: Blob, fileName: string) => {
  const basePath = await downloadDir();
  let selected = await save({
    title: "保存Excel文件",
    defaultPath: basePath
  });
  console.log("selected: ", selected);
  if (!selected) return
  const reader = new FileReader();
  reader.readAsArrayBuffer(blob);
  reader.onload = async (e: any) => {
    let u8 = new Uint8Array(e.target.result);
    await writeBinaryFile(`${selected}${fileName}`, u8);
    // await writeBinaryFile({ contents: u8, path: `${selected}${fileName}` });
    await message(`File saved to ${selected}`);
  };
}

const saveData = async () => {
  const { is_staff, is_graduate, ...restData } = formData;
  const res = await DocCollection.getInstance().addDoc({
    ...restData
  })
};

const onResetForm = () => {
  if (!formRef.value) return;
  formRef.value.resetFields();
};

const validateForm = async () => {
  if (!formRef.value) return;
  return await formRef.value.validate((valid: boolean, fields: any) => {
    if (valid) {
      console.log("submit!");
    } else {
      console.log("error submit!", fields);
    }
    return valid;
  });
};

const exportDoc = async () => {
  const valid = await validateForm();
  if (valid) {
    await saveData();
    exportWord();
  }
};
</script>

<style lang="scss" scoped>
@import "./index.scss";
</style>
