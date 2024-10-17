import axios from "axios";
import router from "@/router";
import { message } from "ant-design-vue";

export const instance = axios.create({
  timeout: 300000,
});

const defaultErrMsg = "出现错误，请稍后重试";

instance.interceptors.request.use((config) => {
  return config;
});

instance.interceptors.response.use(
  (response) => {
    if (response.status === 200) {
      // 网络请求成功
      if (response.data.code === 0) {
        // 接口返回成功
        return response.data.result;
      }
      message.error(response.data.msg || defaultErrMsg);
      return Promise.reject(response.data.msg || defaultErrMsg);
    }
    return Promise.reject(response.statusText);
  },
  (error) => {
    message.error(error?.response?.data?.msg || defaultErrMsg);
    return Promise.reject(error);
  }
);

export default instance;
