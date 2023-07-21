<template>
  <div class="container">
    <el-button class="btn-back" type="primary" round @click="goBack">返回上一页</el-button>

    <div class="search">
      <el-input v-model="keyword" placeholder="请输入姓名查询" clearable>
        <template #prepend>
          <el-button :icon="Search" />
        </template>
        <template #append>
          <el-button type="primary" @click="searchFiles">搜索</el-button>
        </template>
      </el-input>
    </div>

    <el-table :data="tableData" border stripe style="width: 100%">
      <el-table-column prop="name" label="姓名" width="180" />
      <el-table-column prop="sex" label="性别" width="180" />
      <el-table-column prop="birthday" label="出生年月" />
      <el-table-column prop="candidate_type" label="考生类别" />
      <el-table-column prop="identity_num" label="身份证号" />
      <el-table-column prop="company_name" label="单位名称" />
      <el-table-column prop="company_type" label="单位性质" />
      <el-table-column prop="contract_time" label="签约时间" />
      <!-- <el-table-column label="编辑操作">
        <template #default="scope">
          <el-button @click="handleEditDoc(scope.$index, scope.row)"
            >编辑</el-button
          >
        </template>
      </el-table-column> -->
    </el-table>

    <div class="page">
      <el-pagination v-model:currentPage="currentPage" v-model:page-size="pageSize" background layout="prev, pager, next"
        :total="allData.length" @current-change="handleCurrentChange" />
    </div>

    <div class="date">
      <div class="label">Excel导出日期范围:</div>
      <el-date-picker v-model="exportDate" type="daterange" range-separator="到" start-placeholder="开始日期"
        end-placeholder="结束日期" value-format="x" @change="onDateChange" />
    </div>

    <el-button class="btn-export" type="primary" @click="exportExcel">导出Excel文档</el-button>

    <div v-if="isAdmin" class="files">
      <el-upload ref="dragFileRef" drag :auto-upload="false" :limit="1" accept=".xls,.xlsx" :on-exceed="onFileExceed"
        :on-change="onFileChange" :on-remove="onFileRemove">
        <el-icon class="el-icon--upload"><upload-filled /></el-icon>
        <div class="el-upload__text">将文件拖放到此处或<em>点击上传</em></div>
        <template #tip>
          <div class="el-upload__tip">文件大小不超过50M</div>
        </template>
      </el-upload>

      <el-button class="btn-import" type="primary" @click="parseExcelFile">导入Excel文档</el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, shallowRef } from "vue";
import { ElMessage } from "element-plus";
import { UploadFilled, Search, Plus } from "@element-plus/icons-vue";

import { saveAs } from "file-saver";
import { read, write, utils, WorkBook } from "xlsx";
import { useRouter } from "vue-router";

import { DocCollection } from "@/utils/doc";
import type { Doc } from "@/utils/doc";

import { DialogFilter, message, open, save } from '@tauri-apps/api/dialog';
import { downloadDir } from '@tauri-apps/api/path';
import { readBinaryFile, writeBinaryFile } from '@tauri-apps/api/fs';

const router = useRouter();

const keyword = ref("");
const exportDate = ref<any>("");
const tableData = ref<any[]>([]);
const dragFileRef = ref(null);
const currentPage = ref(1);
const pageSize = ref(10);
const allData = ref<any[]>([]);
const excelData = shallowRef<any[][]>([[]])

let fileRaw: any = null;
let importExcelData: any[] = [];

const filters: DialogFilter[] = [
  // { name: "Excel Binary Workbook", extensions: ["xlsb"] },
  { name: "Excel Workbook", extensions: ["xlsx"] },
  { name: "Excel 97-2004 Workbook", extensions: ["xls"] },
  // { name: "Excel 2003 XML Spreadsheet", extensions: ["xml"] },
  // { name: "Symbolic Link", extensions: ["slk"] },
  // { name: "Flat OpenDocument Spreadsheet", extensions: ["fods"] },
  // { name: "OpenDocument Spreadsheet", extensions: ["fods"] },
];

const start = computed(() => (currentPage.value - 1) * pageSize.value);
const end = computed(() => start.value + pageSize.value);

const isAdmin = computed(
  () => {
    const loginRes = localStorage.getItem("login")
    if (loginRes) {
      return JSON.parse(loginRes).user_name === "admin"
    } else return false
  }
);

onMounted(() => { });

const handleCurrentChange = (val: number) => {
  console.log(`current page: ${val}`);
  tableData.value = allData.value.slice(start.value, end.value);
};

const searchFiles = async () => {
  currentPage.value = 1;
  const stmt = `name like '%${keyword.value.trim()}%'`
  const res = await DocCollection.getInstance().getDocsByStmt(stmt)
  console.log("searchFiles: ", stmt, res);
  allData.value = [...res];
  tableData.value = allData.value.slice(start.value, end.value);
};

const onDateChange = (val: string) => {
  console.log("onDateChange: ", val);
};

// const handleEditDoc = (index, row) => {
//   console.log(index, row);
// };

const exportExcel = async () => {
  if (exportDate.value === "") {
    ElMessage({
      message: "请先选择导出的日期范围",
      type: "warning",
    });
    return;
  }

  if (tableData.value.length === 0) {
    ElMessage({
      message: "暂无数据导出",
      type: "warning",
    });
    return;
  }

  const startDate: number = (exportDate.value[0]) / 1000;
  const endDate: number = (exportDate.value[1]) / 1000;
  const stmt = `create_time between ${startDate} and ${endDate}`
  const res = await DocCollection.getInstance().getDocsByStmt(stmt)
  const data = res.map((_) => {
    const { id, name, sex, birthday, candidate_type, identity_num, company_name, company_type, contract_time } = _
    return {
      id,
      name,
      sex,
      birthday,
      candidate_type,
      identity_num,
      company_name,
      company_type,
      contract_time
    }
  })
  console.log("exportExcel: ", data);

  const wb = utils.book_new();
  const ws = utils.json_to_sheet(data);
  utils.book_append_sheet(wb, ws);

  utils.sheet_add_aoa(
    ws,
    [
      [
        "序号",
        "姓名",
        "性别",
        "出生年月",
        "考生类别",
        "身份证号",
        "单位名称",
        "单位性质",
        "签约时间"
      ],
    ],
    {
      origin: "A1",
    }
  );

  /* calculate column width */
  const max_width = data.reduce((w, r) => {
    return Math.max(w, r.identity_num.length);
  }, 10);
  ws["!cols"] = [{ wch: max_width }];

  const loginRes = localStorage.getItem("login") || null
  const userName = loginRes ? JSON.parse(loginRes).user_name : '';
  // const wbout = writeFile(wb, fileName);

  try {
    saveExcelFile(wb)
    // FileSaver.saveAs(
    //   new Blob([wbout], { type: "application/octet-stream" }),
    //   `Excel导出数据-${user}.xlsx`
    // );
  } catch (e) {
    console.log(e)
  }
};

const saveExcelFile = async (wb: WorkBook) => {
  try {
    const basePath = await downloadDir();
    // filePath = filePath.replace(/Untitled$/, '');
    // const fileName = `Excel导出数据-${userName}.xlsx`
    /* show save file dialog */
    let selected = await save({
      title: "保存Excel文件",
      defaultPath: basePath,
      filters
    });
    if (!selected) throw new Error("No file selected");
    /* Generate workbook */
    const bookType = selected.slice(selected.lastIndexOf(".") + 1);
    if (!bookType) return
    const d = write(wb, { type: "buffer", bookType: bookType as any }) as Uint8Array;
    /* save data to file */
    await writeBinaryFile(selected, d);
    await message(`File saved to ${selected}`);
  } catch (e) {
    await message((e as Error).message || (e as string), { title: "Save Error", type: "error" });
  }
}

const checkImportData = async () => {
  const res = await DocCollection.getInstance().getAllDocs()
  const noRepeatData = importExcelData.filter((item) => {
    const allDocIds = res.map((el: Doc) => el.id);
    return !allDocIds.includes(item['序号']);
  });

  console.log("checkImportData: ", noRepeatData);

  if (noRepeatData.length > 0) {
    insertImportData(noRepeatData);
  } else {
    ElMessage({
      message: "导入重复数据",
      type: "warning",
    });
  }
};

const insertImportData = async (data: any[]) => {
  const arr = data.map(_ => {
    return {
      id: _['序号'],
      name: _['姓名'],
      sex: _['性别'],
      birthday: _['出生年月'],
      candidate_type: _['考生类别'],
      identity_num: _['身份证号'],
      company_name: _['单位名称'],
      company_type: _['单位性质'],
      contract_time: _['签约时间']
    }
  })
  for (const doc of arr) {
    await DocCollection.getInstance().addDoc(doc)
  }
  ElMessage({
    message: "导入Excel数据成功",
    type: "success",
  });
  searchFiles();
};

const parseExcelFile = () => {
  if (!fileRaw) {
    ElMessage({
      message: "请先选择要导入的Excel文档",
      type: "warning",
    });
    return;
  }

  const fileReader = new FileReader();
  fileReader.readAsBinaryString(fileRaw); //readAsArrayBuffer
  fileReader.onload = (ev) => {
    // console.log("onload: ", ev);
    try {
      const data = ev.target ? ev.target.result : "";
      const workbook = read(data, {
        type: "binary",
      });
      const wsname = workbook.SheetNames[0]; //取第一张表
      const ws = utils.sheet_to_json(workbook.Sheets[wsname]); //生成json表格内容
      importExcelData = ws;
      checkImportData();
    } catch (e) {
      return false;
    }
  };
};

const onFileChange = (file: any, files: any[]) => {
  console.log(file);
  fileRaw = file.raw;
};

const onFileRemove = (file: any, files: any[]) => {
  console.log(file);
};

const onFileExceed = (files: any[], uploadFiles: any[]) => {
  console.log(files, uploadFiles);
  ElMessage({
    message: "每次解析处理一个Excel文件",
    type: "warning",
  });
};

const goBack = () => {
  router.go(-1);
};
</script>

<style lang="scss" scoped>
@import "./index.scss";
</style>
