import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Repo } from "@/types/interfaces";

export const repoList: Ref<Repo[]> = ref<Repo[]>([]);

export const username: Ref<String> = ref("");

export async function fetchRepos(): Promise<void> {
  const repos: Repo[] = await invoke("get_repos", { username: username.value });
  console.log(repos);
  repoList.value = repos;
}

export function handleSubmit(event: Event): void {
  event.preventDefault();
  fetchRepos();
}