
<script lang="ts">
	import type { PageData } from "./$types";
	import { URL } from '../constants';
	// Get todos from page load
	export let data: PageData;
	let todos = data.todos;
	
	
  
	// Delete todo
	async function deleteTodo(id: number) {
	  await fetch(`${URL}/delete/${id}`, { method: "DELETE" });
	  todos = todos.filter((todo) => todo.id !== id);
	}
  
// 	async function updateTodo(todo: Todo) {
// 		try {
// 			const response = await fetch(`${URL}/update`, {
// 			method: "PUT",
// 			headers: {
// 				"Content-Type": "application/x-www-form-urlencoded",
// 			},
// 			body: new URLSearchParams({
// 				"id": todo.id.toString(),
//         		"description": todo.description,
//         		"done": todo.done.toString(),
// 			}),
// 			});

// 			if (!response.ok) {
// 			throw new Error(`HTTP error! Status: ${response.status}`);
// 			}

// 			// Redirecting to the provided location after successful update
// 			// Assuming that the backend sends a 'Location' header with the redirect URL
// 			const location = response.headers.get('Location');
// 			if (location) {
// 			window.location.href = location;
// 			} else {
// 			console.log('Todo updated successfully, but no redirect location provided.');
// 			}
// 		} catch (error) {
// 			console.error('Failed to update todo:', error);
// 		}
// }
async function updateTodo(todo: Todo) {
  try {
    const response = await fetch(`${URL}/update`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(todo),
    });

    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }

    // Redirecting to the provided location after successful update
    const location = response.headers.get('Location');
    if (location) {
      window.location.href = location;
    } else {
      console.log('Todo updated successfully, but no redirect location provided.');
    }
  } catch (error) {
    console.error('Failed to update todo:', error);
  }
}	
	

  </script>
<div class="container mx-auto mt-16">
	<h1 class="h1 text-center">Todos</h1>
	
	<div class="max-w-screen-md mx-auto">
	  <form action={`${URL}/create`} method="POST">
		<input
		  class="input p-4 my-8"
		  name="description"
		  type="text"
		  placeholder="What needs to be done?"
		  autocomplete="off"
		/>
	  </form>
  
	  <div class="space-y-4">
		{#each todos as todo}
		  <div class="flex items-center justify-between p-4 bg-surface-800 rounded-lg gap-4">
			<input
			  class="checkbox"
			  type="checkbox"
			  bind:checked={todo.done}
			  on:change={() => updateTodo(todo)}
			/>
			<input class="input" type="text" bind:value={todo.description} disabled={todo.done} />
  
			<div class="flex gap-2">
			  <button class="btn variant-filled-secondary" on:click={() => updateTodo(todo)}>Update</button>
			  <button class="btn variant-filled-primary" on:click={() => deleteTodo(todo.id)}>Delete</button
			  >
			</div>
		  </div>
		{/each}
	  </div>
	</div>
  </div>