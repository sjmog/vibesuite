import { useState, useEffect, useCallback } from 'react';
import { Button } from '@/components/ui/button';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Input } from '@/components/ui/input';
import { FileSearchTextarea } from '@/components/ui/file-search-textarea';
import { Label } from '@/components/ui/label';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { useConfig } from '@/components/config-provider';
import { makeRequest } from '@/lib/api';
import type { TaskStatus, ExecutorConfig, ProjectPersonaWithTemplate, ApiResponse } from 'shared/types';

interface Task {
  id: string;
  project_id: string;
  title: string;
  description: string | null;
  status: TaskStatus;
  created_at: string;
  updated_at: string;
  assigned_persona_id: string | null;
}

interface TaskFormDialogProps {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  task?: Task | null; // Optional for create mode
  projectId?: string; // For file search functionality
  onCreateTask?: (title: string, description: string, assignedPersonaId: string | null) => Promise<void>;
  onCreateAndStartTask?: (
    title: string,
    description: string,
    executor?: ExecutorConfig,
    assignedPersonaId?: string | null
  ) => Promise<void>;
  onUpdateTask?: (
    title: string,
    description: string,
    status: TaskStatus,
    assignedPersonaId: string | null
  ) => Promise<void>;
}

export function TaskFormDialog({
  isOpen,
  onOpenChange,
  task,
  projectId,
  onCreateTask,
  onCreateAndStartTask,
  onUpdateTask,
}: TaskFormDialogProps) {
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [status, setStatus] = useState<TaskStatus>('todo');
  const [assignedPersonaId, setAssignedPersonaId] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isSubmittingAndStart, setIsSubmittingAndStart] = useState(false);
  const [personas, setPersonas] = useState<ProjectPersonaWithTemplate[]>([]);
  const [loadingPersonas, setLoadingPersonas] = useState(false);

  const { config } = useConfig();
  const isEditMode = Boolean(task);

  useEffect(() => {
    if (task) {
      // Edit mode - populate with existing task data
      setTitle(task.title);
      setDescription(task.description || '');
      setStatus(task.status);
      setAssignedPersonaId(task.assigned_persona_id);
    } else {
      // Create mode - reset to defaults
      setTitle('');
      setDescription('');
      setStatus('todo');
      setAssignedPersonaId(null);
    }
  }, [task, isOpen]);

  // Fetch personas when dialog opens
  useEffect(() => {
    if (isOpen && projectId) {
      fetchPersonas();
    }
  }, [isOpen, projectId]);

  const fetchPersonas = async () => {
    if (!projectId) return;
    
    setLoadingPersonas(true);
    try {
      const response = await makeRequest(`/api/personas/projects/${projectId}/personas`);
      const data: ApiResponse<ProjectPersonaWithTemplate[]> = await response.json();
      
      if (data.success && data.data) {
        const activePersonas = data.data.filter(p => p.is_active);
        setPersonas(activePersonas);
        
        // Set default persona to PM if available and not in edit mode
        if (!task && assignedPersonaId === null && activePersonas.length > 0) {
          // Look for PM persona by checking template name or custom name
          const pmPersona = activePersonas.find(p => 
            p.template_name?.toLowerCase().includes('pm') ||
            p.template_name?.toLowerCase().includes('project manager') ||
            p.custom_name?.toLowerCase().includes('pm') ||
            p.custom_name?.toLowerCase().includes('project manager')
          );
          
          if (pmPersona) {
            setAssignedPersonaId(pmPersona.id);
          }
        }
      }
    } catch (err) {
      console.error('Failed to load personas:', err);
    } finally {
      setLoadingPersonas(false);
    }
  };

  const handleSubmit = async () => {
    if (!title.trim()) return;

    setIsSubmitting(true);
    try {
      if (isEditMode && onUpdateTask) {
        await onUpdateTask(title, description, status, assignedPersonaId);
      } else if (!isEditMode && onCreateTask) {
        await onCreateTask(title, description, assignedPersonaId);
      }

      // Reset form on successful creation
      if (!isEditMode) {
        setTitle('');
        setDescription('');
        setStatus('todo');
        setAssignedPersonaId(null);
      }

      onOpenChange(false);
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleCreateAndStart = useCallback(async () => {
    if (!title.trim()) return;

    setIsSubmittingAndStart(true);
    try {
      if (!isEditMode && onCreateAndStartTask) {
        await onCreateAndStartTask(title, description, config?.executor, assignedPersonaId);
      }

      // Reset form on successful creation
      setTitle('');
      setDescription('');
      setStatus('todo');
      setAssignedPersonaId(null);

      onOpenChange(false);
    } finally {
      setIsSubmittingAndStart(false);
    }
  }, [
    title,
    description,
    config?.executor,
    assignedPersonaId,
    isEditMode,
    onCreateAndStartTask,
    onOpenChange,
  ]);

  const handleCancel = useCallback(() => {
    // Reset form state when canceling
    if (task) {
      setTitle(task.title);
      setDescription(task.description || '');
      setStatus(task.status);
      setAssignedPersonaId(task.assigned_persona_id);
    } else {
      setTitle('');
      setDescription('');
      setStatus('todo');
      setAssignedPersonaId(null);
    }
    onOpenChange(false);
  }, [task, onOpenChange]);

  // Handle keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      // ESC to close dialog (prevent it from reaching TaskDetailsPanel)
      if (event.key === 'Escape') {
        event.preventDefault();
        event.stopPropagation();
        handleCancel();
        return;
      }

      // Command/Ctrl + Enter to Create & Start (create mode) or Save (edit mode)
      if ((event.metaKey || event.ctrlKey) && event.key === 'Enter') {
        if (
          !isEditMode &&
          onCreateAndStartTask &&
          title.trim() &&
          !isSubmitting &&
          !isSubmittingAndStart
        ) {
          event.preventDefault();
          handleCreateAndStart();
        } else if (
          isEditMode &&
          title.trim() &&
          !isSubmitting &&
          !isSubmittingAndStart
        ) {
          event.preventDefault();
          handleSubmit();
        }
      }
    };

    if (isOpen) {
      document.addEventListener('keydown', handleKeyDown, true); // Use capture phase to get priority
      return () => document.removeEventListener('keydown', handleKeyDown, true);
    }
  }, [
    isOpen,
    isEditMode,
    onCreateAndStartTask,
    title,
    isSubmitting,
    isSubmittingAndStart,
    handleCreateAndStart,
    handleCancel,
  ]);

  return (
    <Dialog open={isOpen} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>
            {isEditMode ? 'Edit Task' : 'Create New Task'}
          </DialogTitle>
        </DialogHeader>
        <div className="space-y-4">
          <div>
            <Label htmlFor="task-title">Title</Label>
            <Input
              id="task-title"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              placeholder="Enter task title"
              disabled={isSubmitting || isSubmittingAndStart}
            />
          </div>

          <div>
            <Label htmlFor="task-description">Description</Label>
            <FileSearchTextarea
              value={description}
              onChange={setDescription}
              placeholder="Enter task description (optional). Type @ to search files."
              rows={3}
              maxRows={8}
              disabled={isSubmitting || isSubmittingAndStart}
              projectId={projectId}
            />
          </div>

          <div>
            <Label htmlFor="task-persona">Assign to Persona</Label>
            <Select
              value={assignedPersonaId || 'unassigned'}
              onValueChange={(value) => setAssignedPersonaId(value === 'unassigned' ? null : value)}
              disabled={isSubmitting || isSubmittingAndStart || loadingPersonas}
            >
              <SelectTrigger>
                <SelectValue placeholder="Select a persona" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="unassigned">Unassigned</SelectItem>
                {personas.map((persona) => (
                  <SelectItem key={persona.id} value={persona.id}>
                    {persona.custom_name || persona.template_name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {loadingPersonas && (
              <p className="text-sm text-muted-foreground mt-1">Loading personas...</p>
            )}
          </div>

          {isEditMode && (
            <div>
              <Label htmlFor="task-status">Status</Label>
              <Select
                value={status}
                onValueChange={(value) => setStatus(value as TaskStatus)}
                disabled={isSubmitting || isSubmittingAndStart}
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="todo">To Do</SelectItem>
                  <SelectItem value="inprogress">In Progress</SelectItem>
                  <SelectItem value="inreview">In Review</SelectItem>
                  <SelectItem value="done">Done</SelectItem>
                  <SelectItem value="cancelled">Cancelled</SelectItem>
                </SelectContent>
              </Select>
            </div>
          )}

          <div className="flex justify-end space-x-2">
            <Button
              variant="outline"
              onClick={handleCancel}
              disabled={isSubmitting || isSubmittingAndStart}
            >
              Cancel
            </Button>
            {isEditMode ? (
              <Button
                onClick={handleSubmit}
                disabled={isSubmitting || !title.trim()}
              >
                {isSubmitting ? 'Updating...' : 'Update Task'}
              </Button>
            ) : (
              <>
                <Button
                  variant="outline"
                  onClick={handleSubmit}
                  disabled={
                    isSubmitting || isSubmittingAndStart || !title.trim()
                  }
                >
                  {isSubmitting ? 'Creating...' : 'Create Task'}
                </Button>
                {onCreateAndStartTask && (
                  <Button
                    onClick={handleCreateAndStart}
                    disabled={
                      isSubmitting || isSubmittingAndStart || !title.trim()
                    }
                  >
                    {isSubmittingAndStart
                      ? 'Creating & Starting...'
                      : 'Create & Start'}
                  </Button>
                )}
              </>
            )}
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
